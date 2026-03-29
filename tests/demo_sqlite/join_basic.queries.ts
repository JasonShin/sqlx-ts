export type SelectItemsWithInventoryParams = [number | null];

export interface ISelectItemsWithInventoryResult {
	inventory_quantity: number | null;
	items_id: number;
	items_name: string;
	items_rarity: string | null;
}

export interface ISelectItemsWithInventoryQuery {
	params: SelectItemsWithInventoryParams;
	result: ISelectItemsWithInventoryResult;
}
