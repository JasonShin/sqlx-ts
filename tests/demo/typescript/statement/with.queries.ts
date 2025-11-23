export type WithSqlParams = [any];

export interface IWithSqlResult {
	id: number;
}

export interface IWithSqlQuery {
	params: WithSqlParams;
	result: IWithSqlResult;
}
