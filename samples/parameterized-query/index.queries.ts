

export type SampleQueryParams = [];


export interface ISampleQueryResult {
    id: number;
	points: number;
};


export interface ISampleQueryQuery {
    params: SampleQueryParams;
    result: ISampleQueryResult;
};

