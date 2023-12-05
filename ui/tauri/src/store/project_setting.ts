import { create } from 'zustand';

import { projectSetting } from '/@/settings/project_setting';

export interface ProjectSettingStore {
  sliderWidth: number;
  setSliderWidth: (newSliderWidth: number) => void;
}

const useProjectSettingStore = create<ProjectSettingStore>()((set) => ({
  sliderWidth: projectSetting.sliderWidth,
  setSliderWidth: (newSliderWidth) =>
    set(() => ({ sliderWidth: newSliderWidth })),
}));

export default useProjectSettingStore;
