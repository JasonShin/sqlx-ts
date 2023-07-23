

export type AwaitClientQueryParams = [];


export interface IAwaitClientQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IAwaitClientQueryQuery {
    params: AwaitClientQueryParams;
    result: IAwaitClientQueryResult;
};




export type GetItemsWithRowsParams = [];


export interface IGetItemsWithRowsResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IGetItemsWithRowsQuery {
    params: GetItemsWithRowsParams;
    result: IGetItemsWithRowsResult;
};

