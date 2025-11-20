export type BasicFullOuterJoinParams = [];

export interface IBasicFullOuterJoinResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IBasicFullOuterJoinQuery {
	params: BasicFullOuterJoinParams;
	result: IBasicFullOuterJoinResult;
}

export type FullOuterJoinWithCoalesceParams = [];

export interface IFullOuterJoinWithCoalesceResult {
	id: number;
	inventoryQuantity: number | null;
	itemsName: string;
}

export interface IFullOuterJoinWithCoalesceQuery {
	params: FullOuterJoinWithCoalesceParams;
	result: IFullOuterJoinWithCoalesceResult;
}
