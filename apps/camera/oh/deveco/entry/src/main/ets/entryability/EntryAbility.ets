import { AbilityConstant, UIAbility, Want } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { window } from '@kit.ArkUI';
import { BusinessError } from '@kit.BasicServicesKit';
import { Glue } from '../camera/glue';
import camera from 'libcamera_oh.so';

/** The ability shell: everything else is Rust over the ArkUI C API (pages/Index.ets owns one NodeContent). */
export default class EntryAbility extends UIAbility {
  onCreate(_want: Want, _launchParam: AbilityConstant.LaunchParam): void {
    Glue.initInstance(this.context);
    hilog.info(0x0000, 'camera', 'Ability onCreate');
  }
  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent('pages/Index', (err) => {
      if (err.code) { hilog.error(0x0000, 'camera', 'loadContent failed: %{public}s', JSON.stringify(err) ?? ''); }
    });
    // Edge to edge like the real camera: black behind a transparent status bar, no navigation
    // indicator, and the artboard (406x776 vp) laid out below the status bar's avoid area.
    windowStage.getMainWindow().then((mainWindow) => {
      mainWindow.setWindowLayoutFullScreen(true).catch((e: BusinessError) => {
        hilog.warn(0x0000, 'camera', 'layout full screen failed: %{public}d', e.code);
      });
      mainWindow.setSpecificSystemBarEnabled('navigationIndicator', false).catch((e: BusinessError) => {
        hilog.warn(0x0000, 'camera', 'hide navigation indicator failed: %{public}d', e.code);
      });
      mainWindow.setWindowSystemBarProperties({
        statusBarColor: '#00000000',
        navigationBarColor: '#00000000',
        statusBarContentColor: '#FFFFFF',
        navigationBarContentColor: '#FFFFFF'
      }).catch((e: BusinessError) => {
        hilog.warn(0x0000, 'camera', 'system bar properties failed: %{public}d', e.code);
      });
      const report = () => {
        try {
          const system = mainWindow.getWindowAvoidArea(window.AvoidAreaType.TYPE_SYSTEM);
          camera.setTopInset(px2vp(system.topRect.height));
        } catch (e) {
          hilog.warn(0x0000, 'camera', 'avoid area query failed: %{public}s', JSON.stringify(e));
        }
      };
      mainWindow.on('avoidAreaChange', () => report());
      report();
    }).catch((e: BusinessError) => {
      hilog.warn(0x0000, 'camera', 'main window unavailable: %{public}d', e.code);
    });
  }
  onDestroy(): void {}
  onWindowStageDestroy(): void {}
  onForeground(): void {}
  onBackground(): void {}
};
