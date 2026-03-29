export type PlaceholderBeforeComparisonParams = [number | null, number | null];

export interface IPlaceholderBeforeComparisonResult {
	character_id: number | null;
	id: number;
	quantity: number | null;
}

export interface IPlaceholderBeforeComparisonQuery {
	params: PlaceholderBeforeComparisonParams;
	result: IPlaceholderBeforeComparisonResult;
}

export type PlaceholderBetweenExprParams = [number | null];

export interface IPlaceholderBetweenExprResult {
	character_id: number | null;
	id: number;
	quantity: number | null;
}

export interface IPlaceholderBetweenExprQuery {
	params: PlaceholderBetweenExprParams;
	result: IPlaceholderBetweenExprResult;
}

export type BetweenPlaceholderBoundsParams = [number | null, number | null];

export interface IBetweenPlaceholderBoundsResult {
	character_id: number | null;
	id: number;
	quantity: number | null;
}

export interface IBetweenPlaceholderBoundsQuery {
	params: BetweenPlaceholderBoundsParams;
	result: IBetweenPlaceholderBoundsResult;
}

export type MixedParamPositionsParams = [number | null, number | null];

export interface IMixedParamPositionsResult {
	character_id: number | null;
	id: number;
	quantity: number | null;
}

export interface IMixedParamPositionsQuery {
	params: MixedParamPositionsParams;
	result: IMixedParamPositionsResult;
}
