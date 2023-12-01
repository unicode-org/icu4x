export 'src/lib.g.dart' hide init, Logger;
import 'src/lib.g.dart' as private show init, Logger;

void init(String path) {
  private.init(path);
  private.Logger.initSimpleLogger();
}