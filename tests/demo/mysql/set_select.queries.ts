export type SetSelect1Params = [];

export interface ISetSelect1Result {
	set1: string | null;
}

export interface ISetSelect1Query {
	params: SetSelect1Params;
	result: ISetSelect1Result;
}

export type SetSelect2Params = [];

export interface ISetSelect2Result {
	intz: number | null;
	set1: string | null;
	varchar1: string | null;
}

export interface ISetSelect2Query {
	params: SetSelect2Params;
	result: ISetSelect2Result;
}

export type SetSelect3Params = [number | null];

export interface ISetSelect3Result {
	set1: string | null;
}

export interface ISetSelect3Query {
	params: SetSelect3Params;
	result: ISetSelect3Result;
}
