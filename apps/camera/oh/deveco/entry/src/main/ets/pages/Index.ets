import { NodeContent } from '@kit.ArkUI';
import camera from 'libcamera_oh.so';
import { Glue } from '../camera/glue';

/** One slot for the native tree; a 16 ms clock for its motion and its requests. */
@Entry
@Component
struct Index {
  private content: NodeContent = new NodeContent();
  private timer: number = -1;

  aboutToAppear(): void {
    camera.mount(this.content, Glue.getInstance().filesDir());
    this.timer = setInterval(() => {
      camera.tick();
      let request: string = camera.nextRequest();
      while (request.length > 0) {
        Glue.getInstance().handle(request);
        request = camera.nextRequest();
      }
    }, 16);
  }
  aboutToDisappear(): void { if (this.timer >= 0) { clearInterval(this.timer); } }
  onBackPress(): boolean { return camera.back(); }

  build() {
    Column() {
      ContentSlot(this.content)
    }
    .width('100%')
    .height('100%')
    .backgroundColor('#000000')
  }
}
