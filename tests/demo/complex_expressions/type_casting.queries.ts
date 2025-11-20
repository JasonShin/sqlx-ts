export type CastFunctionParams = [];

export interface ICastFunctionResult {
	id: number;
	idAsBigint: number;
	idAsFloat: number;
	idAsText: string;
	name: string;
}

export interface ICastFunctionQuery {
	params: CastFunctionParams;
	result: ICastFunctionResult;
}

export type PostgresqlCastingParams = [];

export interface IPostgresqlCastingResult {
	id: number;
	idAsFloat: number;
	idAsText: string;
	idAsVarchar: string;
	name: string;
}

export interface IPostgresqlCastingQuery {
	params: PostgresqlCastingParams;
	result: IPostgresqlCastingResult;
}

export type CastingWithOperationsParams = [];

export interface ICastingWithOperationsResult {
	id: number;
	idDiv3Precise: number;
	idFromConcat: number;
	name: string;
}

export interface ICastingWithOperationsQuery {
	params: CastingWithOperationsParams;
	result: ICastingWithOperationsResult;
}

export type NullCastingParams = [];

export interface INullCastingResult {
	id: number;
	name: string;
	nullInt: number;
	nullText: string;
}

export interface INullCastingQuery {
	params: NullCastingParams;
	result: INullCastingResult;
}
