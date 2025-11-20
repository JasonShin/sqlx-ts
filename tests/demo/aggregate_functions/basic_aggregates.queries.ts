export type CountVariantsParams = [];

export interface ICountVariantsResult {
	distinctRarityCount: number;
	idCount: number;
	nonNullRarityCount: number;
	totalCount: number;
}

export interface ICountVariantsQuery {
	params: CountVariantsParams;
	result: ICountVariantsResult;
}

export type SumAndAvgParams = [];

export interface ISumAndAvgResult {
	avgId: number;
	avgIdRounded: number;
	rarity: string | null;
	sumId: number;
}

export interface ISumAndAvgQuery {
	params: SumAndAvgParams;
	result: ISumAndAvgResult;
}

export type MinAndMaxParams = [];

export interface IMinAndMaxResult {
	maxId: number;
	maxName: number;
	minId: number;
	minName: number;
	rarity: string | null;
}

export interface IMinAndMaxQuery {
	params: MinAndMaxParams;
	result: IMinAndMaxResult;
}

export type StddevAndVarianceParams = [];

export interface IStddevAndVarianceResult {
	rarity: string | null;
	stddevId: any;
	varianceId: any;
}

export interface IStddevAndVarianceQuery {
	params: StddevAndVarianceParams;
	result: IStddevAndVarianceResult;
}

export type MultipleAggregatesParams = [];

export interface IMultipleAggregatesResult {
	avgId: number;
	count: number;
	maxId: number;
	minId: number;
	rarity: string | null;
	sumId: number;
}

export interface IMultipleAggregatesQuery {
	params: MultipleAggregatesParams;
	result: IMultipleAggregatesResult;
}
