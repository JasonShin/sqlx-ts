export type SimpleCaseBasicParams = [];

export interface ISimpleCaseBasicResult {
	id: number;
	name: string;
	rarityCode: any;
}

export interface ISimpleCaseBasicQuery {
	params: SimpleCaseBasicParams;
	result: ISimpleCaseBasicResult;
}

export type SimpleCaseWithNullParams = [];

export interface ISimpleCaseWithNullResult {
	id: number;
	name: string;
	rarityLevel: any;
}

export interface ISimpleCaseWithNullQuery {
	params: SimpleCaseWithNullParams;
	result: ISimpleCaseWithNullResult;
}

export type MultipleCaseExpressionsParams = [];

export interface IMultipleCaseExpressionsResult {
	id: number;
	idRange: any;
	name: string;
	rarityTier: any;
}

export interface IMultipleCaseExpressionsQuery {
	params: MultipleCaseExpressionsParams;
	result: IMultipleCaseExpressionsResult;
}
