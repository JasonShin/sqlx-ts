export type BasicRightJoinParams = [];

export interface IBasicRightJoinResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IBasicRightJoinQuery {
	params: BasicRightJoinParams;
	result: IBasicRightJoinResult;
}

export type RightJoinWithParamsParams = [number | null];

export interface IRightJoinWithParamsResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IRightJoinWithParamsQuery {
	params: RightJoinWithParamsParams;
	result: IRightJoinWithParamsResult;
}
