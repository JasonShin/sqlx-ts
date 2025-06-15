export type ModuleSqlParams = [];

export interface IModuleSqlResult {
	id: number;
}

export interface IModuleSqlQuery {
	params: ModuleSqlParams;
	result: IModuleSqlResult;
}

