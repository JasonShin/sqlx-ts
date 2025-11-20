export type AndOperatorParams = [];

export interface IAndOperatorResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IAndOperatorQuery {
	params: AndOperatorParams;
	result: IAndOperatorResult;
}

export type OrOperatorParams = [];

export interface IOrOperatorResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IOrOperatorQuery {
	params: OrOperatorParams;
	result: IOrOperatorResult;
}

export type NotOperatorParams = [];

export interface INotOperatorResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface INotOperatorQuery {
	params: NotOperatorParams;
	result: INotOperatorResult;
}

export type CombinedBooleanLogicParams = [];

export interface ICombinedBooleanLogicResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ICombinedBooleanLogicQuery {
	params: CombinedBooleanLogicParams;
	result: ICombinedBooleanLogicResult;
}

export type NestedConditionsParams = [];

export interface INestedConditionsResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface INestedConditionsQuery {
	params: NestedConditionsParams;
	result: INestedConditionsResult;
}

export type BooleanInSelectParams = [];

export interface IBooleanInSelectResult {
	id: number;
	isHighIdRare: string;
	isPremium: boolean;
	name: string;
	rarity: string | null;
}

export interface IBooleanInSelectQuery {
	params: BooleanInSelectParams;
	result: IBooleanInSelectResult;
}

