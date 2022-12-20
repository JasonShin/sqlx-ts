

export type SampleQueryParams = [number, number, number];


export interface ISampleQueryResult {
    points: number;
	id: number;
};


export interface ISampleQueryQuery {
    params: SampleQueryParams;
    result: ISampleQueryResult;
};

