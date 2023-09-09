

export type WhileSqlParams = [boolean];


export interface IWhileSqlResult {
    id: number;
};


export interface IWhileSqlQuery {
    params: WhileSqlParams;
    result: IWhileSqlResult;
};


