export type JsonOperatorsSelectParams = [];

export interface IJsonOperatorsSelectResult {
	extractedName: any;
	id: number;
	name: string;
}

export interface IJsonOperatorsSelectQuery {
	params: JsonOperatorsSelectParams;
	result: IJsonOperatorsSelectResult;
}
