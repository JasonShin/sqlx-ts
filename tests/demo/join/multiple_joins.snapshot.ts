export type MixedJoinTypesParams = [];

export interface IMixedJoinTypesResult {
	itemsId: number;
	itemsName: string;
}

export interface IMixedJoinTypesQuery {
	params: MixedJoinTypesParams;
	result: IMixedJoinTypesResult;
}

export type SelfJoinParams = [];

export interface ISelfJoinResult {
	itemsId: number;
	itemsName: string;
	relatedName: string;
}

export interface ISelfJoinQuery {
	params: SelfJoinParams;
	result: ISelfJoinResult;
}

export type ThreeWayJoinParams = [number | null];

export interface IThreeWayJoinResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IThreeWayJoinQuery {
	params: ThreeWayJoinParams;
	result: IThreeWayJoinResult;
}

export type ComplexJoinWithParamsParams = [string | null, number | null];

export interface IComplexJoinWithParamsResult {
	inventoryQuantity: number | null;
	itemsId: number;
	itemsName: string;
}

export interface IComplexJoinWithParamsQuery {
	params: ComplexJoinWithParamsParams;
	result: IComplexJoinWithParamsResult;
}
