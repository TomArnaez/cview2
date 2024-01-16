const ICON_LIST = {} as const;

export const ICONS: IconDefinitionType<typeof ICON_LIST> = ICON_LIST;

export type IconSize = undefined | 12 | 16 | 24 | 32;
export type IconName = keyof typeof ICONS;

type IconDefinition = { svg: string; size: IconSize };
type EvaluateType<T> = T extends infer O ? { [K in keyof O]: O[K] } : never;
type IconDefinitionType<T extends Record<string, IconDefinition>> = EvaluateType<{ [key in keyof T]: IconDefinition }>;