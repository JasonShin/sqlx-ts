export type LimitBasicParams = [];

export interface ILimitBasicResult {
	id: number;
	name: string;
}

export interface ILimitBasicQuery {
	params: LimitBasicParams;
	result: ILimitBasicResult;
}

export type OffsetBasicParams = [];

export interface IOffsetBasicResult {
	id: number;
	name: string;
}

export interface IOffsetBasicQuery {
	params: OffsetBasicParams;
	result: IOffsetBasicResult;
}

export type LimitOffsetParams = [];

export interface ILimitOffsetResult {
	id: number;
	name: string;
}

export interface ILimitOffsetQuery {
	params: LimitOffsetParams;
	result: ILimitOffsetResult;
}

export type LimitWithOrderByParams = [];

export interface ILimitWithOrderByResult {
	id: number;
	name: string;
}

export interface ILimitWithOrderByQuery {
	params: LimitWithOrderByParams;
	result: ILimitWithOrderByResult;
}

export type PaginationWithParamsParams = [string | null];

export interface IPaginationWithParamsResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IPaginationWithParamsQuery {
	params: PaginationWithParamsParams;
	result: IPaginationWithParamsResult;
}

export type FetchFirstParams = [];

export interface IFetchFirstResult {
	id: number;
	name: string;
}

export interface IFetchFirstQuery {
	params: FetchFirstParams;
	result: IFetchFirstResult;
}

export type FetchWithOffsetParams = [];

export interface IFetchWithOffsetResult {
	id: number;
	name: string;
}

export interface IFetchWithOffsetQuery {
	params: FetchWithOffsetParams;
	result: IFetchWithOffsetResult;
}
