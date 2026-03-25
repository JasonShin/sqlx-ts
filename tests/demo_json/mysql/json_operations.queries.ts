export type JsonOperatorsSelectParams = [];

export interface IJsonOperatorsSelectResult {
	extractedName: unknown;
	id: number;
	name: string;
}

export interface IJsonOperatorsSelectQuery {
	params: JsonOperatorsSelectParams;
	result: IJsonOperatorsSelectResult;
}
