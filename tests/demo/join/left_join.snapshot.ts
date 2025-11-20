export type BasicLeftJoinParams = [];

export interface IBasicLeftJoinResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IBasicLeftJoinQuery {
	params: BasicLeftJoinParams;
	result: IBasicLeftJoinResult;
}

export type LeftJoinWithWhereParams = [string | null];

export interface ILeftJoinWithWhereResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface ILeftJoinWithWhereQuery {
	params: LeftJoinWithWhereParams;
	result: ILeftJoinWithWhereResult;
}

export type LeftJoinNullHandlingParams = [];

export interface ILeftJoinNullHandlingResult {
	itemsId: number;
	itemsName: string;
	quantity: number;
}

export interface ILeftJoinNullHandlingQuery {
	params: LeftJoinNullHandlingParams;
	result: ILeftJoinNullHandlingResult;
}

export type MultipleLeftJoinsParams = [];

export interface IMultipleLeftJoinsResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IMultipleLeftJoinsQuery {
	params: MultipleLeftJoinsParams;
	result: IMultipleLeftJoinsResult;
}
