export type WhileSqlParams = [boolean];

export interface IWhileSqlResult {
	id: number;
}

export interface IWhileSqlQuery {
	params: WhileSqlParams;
	result: IWhileSqlResult;
}

export type QueryParams = [];

export interface IQueryResult {
	id: number;
}

export interface IQueryQuery {
	params: QueryParams;
	result: IQueryResult;
}

