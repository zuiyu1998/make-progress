import { useProjectSettingStore } from '/@/store';
import { useMemo } from 'react';

export function useSlider() {
  const projectSettingStore = useProjectSettingStore();

  const sliderWidth = useMemo(() => {
    return projectSettingStore.sliderWidth;
  }, [projectSettingStore]);

  return {
    sliderWidth,
  };
}
