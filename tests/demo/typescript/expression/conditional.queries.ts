export type TruthyParams = [];

export interface ITruthyResult {
	id: number;
}

export interface ITruthyQuery {
	params: TruthyParams;
	result: ITruthyResult;
}

export type FalsyParams = [];

export interface IFalsyResult {
	id: number;
}

export interface IFalsyQuery {
	params: FalsyParams;
	result: IFalsyResult;
}
