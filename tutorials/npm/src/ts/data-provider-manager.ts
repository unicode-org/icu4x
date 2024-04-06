import {
    ICU4XDataProvider,
    ICU4XLocale,
    ICU4XLocaleFallbacker
} from 'icu4x';
import * as localeDefault from '../../dist/locales';

export class DataProviderManager {

    private dataProvider: ICU4XDataProvider;
    private fallbacker: ICU4XLocaleFallbacker;
    private loadedLocales: Set<ICU4XLocale> ;

    private constructor() {
        this.loadedLocales = new Set <ICU4XLocale> ();
    }

    public static async create(): Promise <DataProviderManager> {
        const manager = new DataProviderManager();
        await manager.init();
        return manager;
    }

    private async init() {

        const enFilePath = 'dist/en.postcard';
        let enProvider = await this.createDataProviderFromBlob(enFilePath);
        this.loadedLocales.add(ICU4XLocale.create_from_string("en"));

        const unFilePath = 'dist/und.postcard';
        let unProvider = await this.createDataProviderFromBlob(unFilePath);

        let fallbacker: ICU4XLocaleFallbacker = ICU4XLocaleFallbacker.create(unProvider);
        enProvider.enable_locale_fallback_with(fallbacker);

        this.fallbacker = fallbacker;
        this.dataProvider = enProvider;
    }

    private async createDataProviderFromBlob(filePath: string): Promise < ICU4XDataProvider > {
        const blob = await this.readBlobFromFile(filePath);
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const newDataProvider = ICU4XDataProvider.create_from_byte_slice(uint8Array);
        return newDataProvider;
    }

    private async readBlobFromFile(path: string): Promise < Blob > {
        const response = await fetch(path);
        if(!response.ok) {
            throw new Error(`Failed to fetch file: ${response.statusText}`);
        }
        const blob = await response.blob();
        return blob;
    }

    public async loadLocale(newLocale: string): Promise < ICU4XDataProvider > {
        const icu4xLocale = ICU4XLocale.create_from_string(newLocale);
        const newFilePath = `dist/${newLocale}.postcard`;
        let newProvider = await this.createDataProviderFromBlob(newFilePath);
        await this.dataProvider.fork_by_locale(newProvider);
        this.dataProvider = newProvider;
        this.loadedLocales.add(ICU4XLocale.create_from_string(icu4xLocale));
        return newProvider;
    }

    public async getSegmenterProviderLocale () : Promise <ICU4XDataProvider> {
        const segmenterLocale = ['ja', 'zh', 'th'];
        let segmenterProvider: ICU4XDataProvider;
        for(let i = 0 ; i < segmenterLocale.length ; i++){
            segmenterProvider = await this.loadLocale(segmenterLocale[i]);
        }
        return segmenterProvider;
    }

    public async getDeDataProvider() {
        const newFilePath = `dist/de.postcard`;
        let newProvider = await this.createDataProviderFromBlob(newFilePath);
        return newProvider;
    }

    public getLoadedLocales() {
        return this.loadedLocales;
    }

    public getDataProvider(): ICU4XDataProvider {
        return this.dataProvider;
    }

    public getFallbacker(): ICU4XLocaleFallbacker {
        return this.fallbacker;
    }

}