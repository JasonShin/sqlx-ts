export type SelectWithParamsParams = [string | null];

export interface ISelectWithParamsResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ISelectWithParamsQuery {
	params: SelectWithParamsParams;
	result: ISelectWithParamsResult;
}

