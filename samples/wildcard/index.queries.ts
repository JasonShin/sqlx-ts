

export interface IWildcardQueryParams {
    
};


export interface IWildcardQueryResult {
    number: number;
	id: number;
	food_type: string;
	points: number | null;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IWildcardQueryQuery {
    params: IWildcardQueryParams;
    result: IWildcardQueryResult;
};

