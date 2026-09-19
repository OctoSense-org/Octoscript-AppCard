import common from '@ohos.app.ability.common';
import abilityAccessCtrl, { Permissions } from '@ohos.abilityAccessCtrl';
import photoAccessHelper from '@ohos.file.photoAccessHelper';
import fileUri from '@ohos.file.fileuri';
import fs from '@ohos.file.fs';
import camera from 'libcamera_oh.so';

declare type Any = ESObject;

/**
 * What the native app cannot do from C on this API level: the user-grant
 * permission prompts and the gallery write. Rust asks for them through the
 * request list `camera.tick()` returns; the answers go back over napi.
 */
export class Glue {
  private static instance: Glue;
  static initInstance(context: common.Context): Glue {
    if (Glue.instance == null) { Glue.instance = new Glue(context); }
    return Glue.instance;
  }
  static getInstance(): Glue { return Glue.instance; }

  context: common.Context;
  constructor(context: common.Context) { this.context = context; }
  filesDir(): string { return this.context.filesDir; }

  /** Perform one request from the native side: "kind|value". */
  handle(request: string): void {
    const sep = request.indexOf('|');
    const kind = sep < 0 ? request : request.substring(0, sep);
    const value = sep < 0 ? '' : request.substring(sep + 1);
    if (kind === 'permission') { this.requestPermission(value); }
    else if (kind === 'save') { this.saveToGallery(value); }
  }

  /** One request may name several permissions ("camera,microphone,gallery"): the system shows them as one flow. */
  private requestPermission(names: string): void {
    const list: Array<string> = names.split(',');
    const permissions: Array<Permissions> = list.map((name: string): Permissions =>
      name === 'microphone' ? 'ohos.permission.MICROPHONE' : name === 'gallery' ? 'ohos.permission.WRITE_IMAGEVIDEO' : 'ohos.permission.CAMERA');
    abilityAccessCtrl.createAtManager().requestPermissionsFromUser(this.context, permissions).then((data) => {
      const results: Array<number> = data.authResults ?? [];
      for (let i = 0; i < list.length; i++) { camera.handlePermissionResult(list[i], i < results.length && results[i] === 0); }
    }).catch((err: Any) => {
      console.error('permissions ' + names + ' failed: ' + JSON.stringify(err));
      for (const name of list) { camera.handlePermissionResult(name, false); }
    });
  }

  private saveToGallery(path: string): void {
    const ext = path.endsWith('.mp4') ? 'mp4' : 'jpg';
    const photoType = ext === 'mp4' ? photoAccessHelper.PhotoType.VIDEO : photoAccessHelper.PhotoType.IMAGE;
    const helper = photoAccessHelper.getPhotoAccessHelper(this.context);
    abilityAccessCtrl.createAtManager().requestPermissionsFromUser(this.context, ['ohos.permission.WRITE_IMAGEVIDEO']).then((data) => {
      const results: Array<number> = data.authResults ?? [];
      if (results.length > 0 && results[0] === 0) {
        helper.createAsset(photoType, ext).then((uri: string) => this.copy(path, uri)).catch((err: Any) => {
          console.error('createAsset failed, using the dialog: ' + JSON.stringify(err)); this.saveWithDialog(helper, path, ext, photoType);
        });
      } else { this.saveWithDialog(helper, path, ext, photoType); }
    }).catch(() => this.saveWithDialog(helper, path, ext, photoType));
  }

  private saveWithDialog(helper: photoAccessHelper.PhotoAccessHelper, path: string, ext: string, photoType: photoAccessHelper.PhotoType): void {
    const config: photoAccessHelper.PhotoCreationConfig = { fileNameExtension: ext, photoType: photoType };
    helper.showAssetsCreationDialog([fileUri.getUriFromPath(path)], [config]).then((uris: Array<string>) => {
      if (uris.length === 0 || uris[0].length === 0) { camera.handleCaptureSaved(path, false); return; }
      this.copy(path, uris[0]);
    }).catch((err: Any) => { console.error('gallery dialog failed: ' + JSON.stringify(err)); camera.handleCaptureSaved(path, false); });
  }

  private copy(path: string, uri: string): void {
    try {
      const target = fs.openSync(uri, fs.OpenMode.WRITE_ONLY);
      const source = fs.openSync(path, fs.OpenMode.READ_ONLY);
      fs.copyFileSync(source.fd, target.fd);
      fs.closeSync(source); fs.closeSync(target);
      camera.handleCaptureSaved(path, true);
    } catch (err) { console.error('copy to gallery failed: ' + JSON.stringify(err)); camera.handleCaptureSaved(path, false); }
  }
}
