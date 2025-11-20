export type CountWithFilterParams = [];

export interface ICountWithFilterResult {
	countAbove5: number;
	countAtOrBelow5: number;
	rarity: string | null;
}

export interface ICountWithFilterQuery {
	params: CountWithFilterParams;
	result: ICountWithFilterResult;
}

export type SumWithFilterParams = [];

export interface ISumWithFilterResult {
	rarity: string | null;
	sumAbove5: number;
	sumAtOrBelow5: number;
}

export interface ISumWithFilterQuery {
	params: SumWithFilterParams;
	result: ISumWithFilterResult;
}

export type AvgWithFilterParams = [];

export interface IAvgWithFilterResult {
	avgAbove5: number;
	rarity: string | null;
}

export interface IAvgWithFilterQuery {
	params: AvgWithFilterParams;
	result: IAvgWithFilterResult;
}

export type MultipleFiltersParams = [];

export interface IMultipleFiltersResult {
	countCommon: number;
	countLegendary: number;
	countRare: number;
	countUnknown: number;
}

export interface IMultipleFiltersQuery {
	params: MultipleFiltersParams;
	result: IMultipleFiltersResult;
}

