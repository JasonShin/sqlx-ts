export type WithSqlParams = [boolean];

export interface IWithSqlResult {
	id: number;
}

export interface IWithSqlQuery {
	params: WithSqlParams;
	result: IWithSqlResult;
}

