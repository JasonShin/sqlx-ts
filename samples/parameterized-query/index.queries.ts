

export type SampleQueryParams = [any];


export interface ISampleQueryResult {
    id: number;
	points: number;
};


export interface ISampleQueryQuery {
    params: SampleQueryParams;
    result: ISampleQueryResult;
};

