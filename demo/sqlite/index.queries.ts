export type GetAllItemsParams = [];

export interface IGetAllItemsResult {
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
}

export interface IGetAllItemsQuery {
	params: GetAllItemsParams;
	result: IGetAllItemsResult;
}

export type GetItemsByFoodTypeParams = [string];

export interface IGetItemsByFoodTypeResult {
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
}

export interface IGetItemsByFoodTypeQuery {
	params: GetItemsByFoodTypeParams;
	result: IGetItemsByFoodTypeResult;
}

export type InsertItemParams = [[string, number, number, number]];

export interface IInsertItemResult {
	
}

export interface IInsertItemQuery {
	params: InsertItemParams;
	result: IInsertItemResult;
}

export type UpdateItemParams = [string, number];

export interface IUpdateItemResult {
	
}

export interface IUpdateItemQuery {
	params: UpdateItemParams;
	result: IUpdateItemResult;
}

export type DeleteItemParams = [string];

export interface IDeleteItemResult {
	
}

export interface IDeleteItemQuery {
	params: DeleteItemParams;
	result: IDeleteItemResult;
}

export type GetItemsWithTableParams = [];

export interface IGetItemsWithTableResult {
	items_food_type: string;
	items_id: number;
	table_number: number;
}

export interface IGetItemsWithTableQuery {
	params: GetItemsWithTableParams;
	result: IGetItemsWithTableResult;
}

export type GetEventsWithScoreParams = [number | null];

export interface IGetEventsWithScoreResult {
	description: string | null;
	id: number;
	is_active: boolean;
	metadata: object | null;
	name: string;
	score: number | null;
	start_date: Date | null;
}

export interface IGetEventsWithScoreQuery {
	params: GetEventsWithScoreParams;
	result: IGetEventsWithScoreResult;
}
