import {
    ICU4XDataProvider,
    ICU4XLocale,
    ICU4XLocaleFallbacker,
    ICU4XLocaleFallbackConfig,
    ICU4XLocaleFallbackPriority
} from 'icu4x';
import * as localeDefault from '../../dist/locales';

export class DataProviderManager {

    private dataProvider: ICU4XDataProvider;
    private fallbacker: ICU4XLocaleFallbacker;
    private fallbackLocale: ICU4XLocale;
    private loadedLocales: Set <ICU4XLocale> ;

    private constructor() {
        this.loadedLocales = new Set < ICU4XLocale > ();
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

        const unFilePath = 'dist/en.postcard';
        let unProvider = await this.createDataProviderFromBlob(unFilePath);


        let fallbacker = ICU4XLocaleFallbacker.create(unProvider);
        let fallbackConfig = new ICU4XLocaleFallbackConfig();
        fallbackConfig.priority = ICU4XLocaleFallbackPriority.Language;
        let fallbackerWithConfig = fallbacker.for_config(fallbackConfig);

        this.fallbacker = fallbackerWithConfig;
        enProvider.enable_locale_fallback_with(this.fallbacker);
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

    public async trackLocaleFallback(fallbacker: ICU4XLocaleFallbacker, locale: ICU4XLocale): Promise < string > {
        let localeSet = new Set(localeDefault.default);
        const initialLocale = locale.to_string();
        let fallbackIterator = fallbacker.fallback_for_locale(locale);
        while(fallbackIterator.get().to_string() != 'und') {
            const fallbackLocale = fallbackIterator.get().to_string();
            if(initialLocale != fallbackLocale){
                console.log(`Tracking back to: ${fallbackLocale}`);
            }
            if(localeSet.has(fallbackLocale)) {
                return fallbackLocale;
            }
            fallbackIterator.step();
        }
        return 'und';
    }

    public async loadLocale(newLocale: string): Promise < ICU4XDataProvider > {
        const icu4xLocale = ICU4XLocale.create_from_string(newLocale);
        const fallbackLocale = await this.trackLocaleFallback(this.fallbacker, icu4xLocale);
        const newFilePath = `dist/${fallbackLocale}.postcard`;
        let newProvider = await this.createDataProviderFromBlob(newFilePath);
        await newProvider.fork_by_locale(this.dataProvider);
        this.dataProvider = newProvider;
        this.loadedLocales.add(ICU4XLocale.create_from_string(fallbackLocale));
        this.fallbackLocale = ICU4XLocale.create_from_string(fallbackLocale);
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

    public getFallbackLocale(): ICU4XLocale {
        return this.fallbackLocale;
    }

}