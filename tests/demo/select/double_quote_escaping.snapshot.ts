export type DoubleQuoteTableNameParams = [];

export interface IDoubleQuoteTableNameResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IDoubleQuoteTableNameQuery {
	params: DoubleQuoteTableNameParams;
	result: IDoubleQuoteTableNameResult;
}

export type DoubleQuoteQualifiedNameParams = [];

export interface IDoubleQuoteQualifiedNameResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IDoubleQuoteQualifiedNameQuery {
	params: DoubleQuoteQualifiedNameParams;
	result: IDoubleQuoteQualifiedNameResult;
}

export type DoubleQuoteColumnNamesParams = [number];

export interface IDoubleQuoteColumnNamesResult {
	id: number;
	name: string;
}

export interface IDoubleQuoteColumnNamesQuery {
	params: DoubleQuoteColumnNamesParams;
	result: IDoubleQuoteColumnNamesResult;
}
