export type NestedCaseBasicParams = [];

export interface INestedCaseBasicResult {
	detailedRarity: any;
	id: number;
	name: string;
	rarity: string | null;
}

export interface INestedCaseBasicQuery {
	params: NestedCaseBasicParams;
	result: INestedCaseBasicResult;
}

export type NestedCaseMultipleLevelsParams = [];

export interface INestedCaseMultipleLevelsResult {
	id: number;
	name: string;
	tier: any;
}

export interface INestedCaseMultipleLevelsQuery {
	params: NestedCaseMultipleLevelsParams;
	result: INestedCaseMultipleLevelsResult;
}

