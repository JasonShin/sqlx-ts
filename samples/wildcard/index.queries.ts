

export type WildcardQueryParams = [];


export interface IWildcardQueryResult {
    id: number;
	time_takes_to_cook: number;
	food_type: string;
	table_id: number;
	number: number;
	points: number | null;
};


export interface IWildcardQueryQuery {
    params: WildcardQueryParams;
    result: IWildcardQueryResult;
};

