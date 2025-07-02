// 国际化消息定义
export interface Messages {
  tooltips: {
    avgRate: string
    minRate: string
    maxRate: string
    avgInterval: string
    errorL: string
    errorR: string
    showTrail: string
    logToggle: string
    cleanLog: string
  }
  settings: {
    language: string
    frameRate: string
    logSize: string
    innerDeadzone: string
    outerDeadzone: string
  }
  buttons: {
    showTrail: string
    hideTrail: string
    logOn: string
    logOff: string
    cleanLog: string
    clear: string
  }
}

// 英文消息
export const enMessages: Messages = {
  tooltips: {
    avgRate: 'Average polling rate of the gamepad input',
    minRate: 'Minimum polling rate recorded',
    maxRate: 'Maximum polling rate recorded',
    avgInterval: 'Average time interval between inputs',
    errorL: 'Average error percentage for left stick',
    errorR: 'Average error percentage for right stick',
    showTrail: 'Toggle to show/hide joystick movement trail',
    logToggle: 'Toggle performance data logging on/off (may cause display lag when enabled)',
    cleanLog: 'Clear all recorded performance logs'
  },
  settings: {
    language: 'Language',
    frameRate: 'Frame Rate',
    logSize: 'Log Size',
    innerDeadzone: 'Inner Deadzone',
    outerDeadzone: 'Outer Deadzone'
  },
  buttons: {
    showTrail: 'Show Trail',
    hideTrail: 'Hide Trail', 
    logOn: 'Log On',
    logOff: 'Log Off',
    cleanLog: 'Clean Log',
    clear: 'Clear'
  }
}

// 中文消息
export const zhMessages: Messages = {
  tooltips: {
    avgRate: '手柄输入的平均轮询频率',
    minRate: '记录到的最小轮询频率',
    maxRate: '记录到的最大轮询频率',
    avgInterval: '输入之间的平均时间间隔',
    errorL: '左摇杆的平均误差百分比',
    errorR: '右摇杆的平均误差百分比',
    showTrail: '切换显示/隐藏摇杆移动轨迹',
    logToggle: '开启/关闭性能数据记录(开启后可能导致显示卡顿)',
    cleanLog: '清除所有已记录的性能日志'
  },
  settings: {
    language: '语言',
    frameRate: '帧率',
    logSize: '日志大小',
    innerDeadzone: '内死区',
    outerDeadzone: '外死区'
  },
  buttons: {
    showTrail: '显示轨迹',
    hideTrail: '隐藏轨迹',
    logOn: '记录:开',
    logOff: '记录:关',
    cleanLog: '清除日志',
    clear: '清除'
  }
}

// 语言列表
export const languages = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'zh', name: 'Chinese', nativeName: '中文' }
] as const

export type LanguageCode = typeof languages[number]['code']
