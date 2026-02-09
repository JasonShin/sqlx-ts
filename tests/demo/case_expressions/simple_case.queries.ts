export type SimpleCaseBasicParams = [];

export interface ISimpleCaseBasicResult {
	id: number;
	name: string;
	rarityCode: unknown;
}

export interface ISimpleCaseBasicQuery {
	params: SimpleCaseBasicParams;
	result: ISimpleCaseBasicResult;
}

export type SimpleCaseWithNullParams = [];

export interface ISimpleCaseWithNullResult {
	id: number;
	name: string;
	rarityLevel: unknown;
}

export interface ISimpleCaseWithNullQuery {
	params: SimpleCaseWithNullParams;
	result: ISimpleCaseWithNullResult;
}

export type MultipleCaseExpressionsParams = [];

export interface IMultipleCaseExpressionsResult {
	id: number;
	idRange: unknown;
	name: string;
	rarityTier: unknown;
}

export interface IMultipleCaseExpressionsQuery {
	params: MultipleCaseExpressionsParams;
	result: IMultipleCaseExpressionsResult;
}
