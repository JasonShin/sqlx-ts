export type BasicCrossJoinParams = [];

export interface IBasicCrossJoinResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IBasicCrossJoinQuery {
	params: BasicCrossJoinParams;
	result: IBasicCrossJoinResult;
}

export type CrossJoinWithWhereParams = [string | null];

export interface ICrossJoinWithWhereResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface ICrossJoinWithWhereQuery {
	params: CrossJoinWithWhereParams;
	result: ICrossJoinWithWhereResult;
}
