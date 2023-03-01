export type NumberAlias = number;
export type BoolAlias = boolean;
export type StringAlias = string;
export type Colour =
  | { t: "Red"; content: number }
  | { t: "Green"; content: number }
  | { t: "Blue"; content: undefined };
