export type BasicSelectParams = [];

export interface IBasicSelectResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IBasicSelectQuery {
	params: BasicSelectParams;
	result: IBasicSelectResult;
}
