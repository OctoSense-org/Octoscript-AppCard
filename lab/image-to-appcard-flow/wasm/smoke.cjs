#!/usr/bin/env node
// Real browser/widget smoke; this is not an image-parity or service-reducer gate.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {parseArgs} = require('node:util');
const {values:args} = parseArgs({options:{
  dist:{type:'string'}, card:{type:'string'}, output:{type:'string'}, playwright:{type:'string'},
  browser:{type:'string'}, origin:{type:'string',default:'http://127.0.0.1:8494'},
  'base-path':{type:'string',default:'/wasm/service-cards/'},
  'asset-prefix':{type:'string',default:'http://127.0.0.1:8170/ux-images/'},
  activate:{type:'string'}, 'expected-action':{type:'string'}, disabled:{type:'string'},
}});
for(const name of ['dist','card','output'])if(!args[name])throw Error(`--${name} is required`);
if(args.activate&&!args['expected-action'])throw Error('--activate requires --expected-action');
const {chromium} = args.playwright ? require(path.resolve(args.playwright)) : require('playwright');
const dist=path.resolve(args.dist), card=path.resolve(args.card), output=path.resolve(args.output);
fs.mkdirSync(output); // Refuse to overwrite an earlier evidence directory.
const save=(name,value)=>fs.writeFileSync(path.join(output,name),JSON.stringify(value,null,2)+'\n');
const read=name=>JSON.parse(fs.readFileSync(path.join(card,name),'utf8'));
const hash=file=>crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
const contract=read('contract.json');
const [width,height]=contract.artboard;
if(![width,height].every(x=>Number.isFinite(x)&&x>0))throw Error('Invalid contract artboard');
const appURL=new URL(args['base-path'],args.origin).href;
const assetBase=new URL('./card-assets/',appURL).href;
const build=JSON.parse(fs.readFileSync(path.join(dist,'build.json'),'utf8'));
for(const [name,expected] of Object.entries(build.files)){
  const file=path.resolve(dist,name);
  if(!file.startsWith(dist+path.sep)||hash(file)!==expected)throw Error(`Package hash mismatch: ${name}`);
}
if(hash(path.join(dist,'octosense-wizard.wasm'))!==build.wasm_sha256)throw Error('WASM hash mismatch');

(async()=>{
  const browser=await chromium.launch({headless:true,...(args.browser?{executablePath:path.resolve(args.browser)}:{})});
  const page=await browser.newPage({viewport:{width:Math.ceil(width),height:Math.ceil(height)},deviceScaleFactor:2});
  const logs=[];
  page.on('console',m=>logs.push({type:m.type(),text:m.text()}));
  page.on('pageerror',e=>logs.push({type:'pageerror',text:String(e)}));
  const timeout=setTimeout(async()=>{
    save('watchdog.json',{passed:false,error:'Browser failed to settle within 90 seconds',build_id:build.build_id,wasm_sha256:build.wasm_sha256});
    save('console.json',logs);
    await Promise.race([browser.close(),new Promise(resolve=>setTimeout(resolve,2000))]);
    process.exit(1);
  },90000);
  try{
    const types={'.html':'text/html','.js':'text/javascript','.mjs':'text/javascript','.wasm':'application/wasm','.ttf':'font/ttf','.json':'application/json','.svg':'image/svg+xml','.png':'image/png','.jpg':'image/jpeg'};
    const basePath=new URL(appURL).pathname;
    await page.route(`${args.origin}/**`,async route=>{
      const url=new URL(route.request().url());
      const relative=url.pathname.startsWith(basePath)?decodeURIComponent(url.pathname.slice(basePath.length)):'../outside';
      const file=path.resolve(dist,relative||'index.html');
      if(!file.startsWith(dist+path.sep)||!fs.existsSync(file)||!fs.statSync(file).isFile())return route.fulfill({status:404,body:'Outside package'});
      await route.fulfill({status:200,path:file,contentType:types[path.extname(file)]||'application/octet-stream'});
    });
    await page.goto(appURL);
    await page.waitForFunction(()=>window.__octosense?.ready,null,{timeout:30000});
    const cardSource=fs.readFileSync(path.join(card,'page.card'),'utf8');
    const theme=cardSource.match(/^theme\s+(light|dark)\s*$/m)?.[1]||'light';
    const inputs=['contract.json','page.card','page.data.json','mapping.json',`kit/native/${theme}/kit.json`];
    const payload={id:'portable-smoke',generation:1,width,height,card:cardSource,data:read('page.data.json'),
      kit:read(`kit/native/${theme}/kit.json`),mapping:read('mapping.json'),updates:[]};
    const request=JSON.parse(JSON.stringify(payload).replaceAll(args['asset-prefix'],assetBase));
    await page.evaluate(value=>window.__octosense.mount(value),request);
    await page.waitForFunction(()=>window.__octosense.snapshot?.generation===1,null,{timeout:45000});
    const snapshot=await page.evaluate(()=>window.__octosense.snapshot);
    if(snapshot.id!=='portable-smoke'||snapshot.width!==width||snapshot.height!==height)throw Error('Wrong native request/artboard');
    if(!snapshot.widgets.length)throw Error('No real native widgets');
    if(snapshot.widgets.some(w=>!w.bounds.every(Number.isFinite)||w.bounds[2]<=0||w.bounds[3]<=0))throw Error('Native widget has invalid geometry');
    save('snapshot.json',snapshot);
    await page.locator('canvas').screenshot({path:path.join(output,'native.png')});
    const click=async sourceId=>{
      const widget=snapshot.widgets.find(w=>w.sourceId===sourceId);
      if(!widget)throw Error(`Native control not found: ${sourceId}`);
      const [x,y,w,h]=widget.bounds;
      await page.mouse.click(x+w/2,y+h/2);
      await page.waitForTimeout(250);
    };
    if(args.disabled){
      if(snapshot.widgets.find(w=>w.sourceId===args.disabled)?.enabled!==false)throw Error('Disabled fixture is not natively disabled');
      await click(args.disabled);
      if(await page.evaluate(()=>window.__octosense.events.some(e=>e.type==='octosense:action')))throw Error('Disabled native control activated');
    }
    if(args.activate){
      await click(args.activate);
      const activated=await page.evaluate(sourceId=>window.__octosense.events.some(e=>e.type==='octosense:action'&&e.id==='portable-smoke'&&e.generation===1&&e.sourceId===sourceId),args['expected-action']);
      if(!activated)throw Error('Native KitAction did not match expected source ID');
    }
    const rejected=[];
    for(const src of ['https://foreign.example/card/assets/asset.svg',`${assetBase}../outside/assets/asset.svg`,`${assetBase}%2e%2e/outside/assets/asset.svg`]){
      const result=await page.evaluate(({request,src})=>{
        let changed=false;
        const replace=value=>{if(!value||typeof value!=='object')return;for(const[key,child]of Object.entries(value)){
          if(!changed&&key==='src'&&typeof child==='string'){value[key]=src;changed=true;}else replace(child);
        }};
        replace(request.data);replace(request.kit);
        if(!changed)return {skipped:true,reason:'Fixture has no Image/Svg asset'};
        try{window.__octosense.mount({...request,id:'unsafe-artwork',generation:2});return {rejected:false};}
        catch(error){return {rejected:true,error:String(error)};}
      },{request,src});
      if(!result.skipped&&(!result.rejected||!result.error.includes('same-origin card-assets')))throw Error('Unsafe artwork accepted');
      rejected.push({src,...result});
    }
    const events=await page.evaluate(()=>window.__octosense.events);
    if(events.some(e=>e.type==='octosense:error')||logs.some(e=>['error','pageerror'].includes(e.type)))throw Error('Native/browser runtime errors occurred');
    save('events.json',events);
    save('report.json',{passed:true,gate:'native-wasm-mount-and-action-smoke',imageParityGate:false,
      url:appURL,browser:browser.version(),browserExecutable:args.browser||'Playwright default',
      build_id:build.build_id,wasm_sha256:build.wasm_sha256,cardId:contract.id,artboard:[width,height],
      widgets:snapshot.widgets.length,imageWidgets:snapshot.widgets.filter(w=>w.imageReady===true).length,
      activated:args['expected-action']||null,disabledBlocked:args.disabled||null,rejectedAssets:rejected,
      fixture_sources:Object.fromEntries(inputs.map(name=>[name,hash(path.join(card,name))]))});
    console.log(JSON.stringify({passed:true,evidence:output,build_id:build.build_id}));
  }catch(error){save('error.json',{passed:false,error:String(error),build_id:build.build_id});throw error;}
  finally{clearTimeout(timeout);save('console.json',logs);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
