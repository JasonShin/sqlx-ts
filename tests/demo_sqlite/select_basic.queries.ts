export type SelectAllItemsParams = [];

export interface ISelectAllItemsResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectAllItemsQuery {
	params: SelectAllItemsParams;
	result: ISelectAllItemsResult;
}

export type SelectItemByIdParams = [number];

export interface ISelectItemByIdResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectItemByIdQuery {
	params: SelectItemByIdParams;
	result: ISelectItemByIdResult;
}

export type SelectItemsByNameParams = [string];

export interface ISelectItemsByNameResult {
	id: number;
	name: string;
}

export interface ISelectItemsByNameQuery {
	params: SelectItemsByNameParams;
	result: ISelectItemsByNameResult;
}
