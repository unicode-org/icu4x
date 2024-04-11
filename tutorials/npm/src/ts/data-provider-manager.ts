import {
    ICU4XDataProvider,
    ICU4XLocale,
    ICU4XLocaleFallbacker,
} from 'icu4x';

export class DataProviderManager {

    private dataProvider: ICU4XDataProvider;
    private loadedLocales: Set<ICU4XLocale>;

    private constructor() {
        this.loadedLocales = new Set<ICU4XLocale>();
    }

    public static async create(): Promise<DataProviderManager> {
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
        enProvider.enable_locale_fallback_with(fallbacker);
        this.dataProvider = enProvider;
    }

    private async createDataProviderFromBlob(filePath: string): Promise<ICU4XDataProvider> {
        const blob = await this.readBlobFromFile(filePath);
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const newDataProvider = ICU4XDataProvider.create_from_byte_slice(uint8Array);
        return newDataProvider;
    }

    private async readBlobFromFile(path: string): Promise<Blob> {
        const response = await fetch(path);
        if (!response.ok) {
            throw new Error(`Failed to fetch file: ${response.statusText}`);
        }
        const blob = await response.blob();
        return blob;
    }

    public supportsLocale(locid: string): boolean {
        const locales = this.getLoadedLocales();
        const localesFinal: string[] = [];
        locales.forEach((item: ICU4XLocale) => {
            localesFinal.push(item.to_string)
        })
        const loadedLocales = new Set(localesFinal);
        return loadedLocales.has(locid);
    }

    public async loadLocale(newLocale: string): Promise<ICU4XDataProvider> {
        const icu4xLocale = ICU4XLocale.create_from_string(newLocale);
        const newFilePath = `dist/${newLocale}.postcard`;
        let newProvider = await this.createDataProviderFromBlob(newFilePath);
        await this.dataProvider.fork_by_locale(newProvider);
        this.loadedLocales.add(ICU4XLocale.create_from_string(newLocale));
        return this.dataProvider;
    }

    public async getSegmenterProviderLocale(): Promise<ICU4XDataProvider> {
        const segmenterLocale = ['ja', 'zh', 'th'];
        let segmenterProvider: ICU4XDataProvider;
        for (let i = 0; i < segmenterLocale.length; i++) {
            segmenterProvider = await this.loadLocale(segmenterLocale[i]);
        }
        return segmenterProvider;
    }

    public getLoadedLocales() {
        return this.loadedLocales;
    }

    public getDataProvider(): ICU4XDataProvider {
        return this.dataProvider;
    }

}