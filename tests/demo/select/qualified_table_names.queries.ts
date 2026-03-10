export type QualifiedTableNameParams = [];

export interface IQualifiedTableNameResult {
	racesId: number;
	racesName: 'human' | 'night elf' | 'dwarf' | 'gnome' | 'orc' | 'troll' | 'tauren' | 'undead';
}

export interface IQualifiedTableNameQuery {
	params: QualifiedTableNameParams;
	result: IQualifiedTableNameResult;
}

export type QualifiedTableNameJoinParams = [];

export interface IQualifiedTableNameJoinResult {
	factionId: number;
	factionName: 'alliance' | 'horde';
	raceId: number;
	raceName: 'human' | 'night elf' | 'dwarf' | 'gnome' | 'orc' | 'troll' | 'tauren' | 'undead';
}

export interface IQualifiedTableNameJoinQuery {
	params: QualifiedTableNameJoinParams;
	result: IQualifiedTableNameJoinResult;
}

export type MixedQualifiedNamesParams = [];

export interface IMixedQualifiedNamesResult {
	factionId: number;
	factionName: 'alliance' | 'horde';
	raceId: number;
	raceName: 'human' | 'night elf' | 'dwarf' | 'gnome' | 'orc' | 'troll' | 'tauren' | 'undead';
}

export interface IMixedQualifiedNamesQuery {
	params: MixedQualifiedNamesParams;
	result: IMixedQualifiedNamesResult;
}
