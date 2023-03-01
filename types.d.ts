type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;
export type NumberAlias = number;

export type BoolAlias = boolean;

export type StringAlias = string;

export type Colour =
  | { t: "Red"; content: number }
  | { t: "Green"; content: number }
  | { t: "Blue"; content: [number, string] };
export interface Person {
  name: string;
  age: number;
  enjoys_coffee: [number, string];
}
