export type BasicRowNumberParams = [];

export interface IBasicRowNumberResult {
	id: number;
	name: string;
	rarity: string | null;
	rowNum: any;
}

export interface IBasicRowNumberQuery {
	params: BasicRowNumberParams;
	result: IBasicRowNumberResult;
}

export type RowNumberWithPartitionParams = [];

export interface IRowNumberWithPartitionResult {
	id: number;
	name: string;
	rarity: string | null;
	rowNum: any;
}

export interface IRowNumberWithPartitionQuery {
	params: RowNumberWithPartitionParams;
	result: IRowNumberWithPartitionResult;
}

export type RowNumberWithWhereParams = [];

export interface IRowNumberWithWhereResult {
	id: number;
	name: string;
	rarity: string | null;
	rowNum: any;
}

export interface IRowNumberWithWhereQuery {
	params: RowNumberWithWhereParams;
	result: IRowNumberWithWhereResult;
}

export type RowNumberWithParamsParams = [string | null];

export interface IRowNumberWithParamsResult {
	id: number;
	name: string;
	rarity: string | null;
	rowNum: any;
}

export interface IRowNumberWithParamsQuery {
	params: RowNumberWithParamsParams;
	result: IRowNumberWithParamsResult;
}
