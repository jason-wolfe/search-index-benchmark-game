import com.eclipsesource.json.Json;
import com.eclipsesource.json.JsonObject;
import org.apache.lucene.document.*;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.store.FSDirectory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Path;
import java.nio.file.Paths;

public class BuildIndex {
    public static void main(String[] args) throws IOException {
        final Path outputPath = Paths.get(args[0]);

        final IndexWriterConfig config = new IndexWriterConfig();
        try (IndexWriter writer = new IndexWriter(FSDirectory.open(outputPath), config)) {
            try (BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in))) {
                final Document document = new Document();

                String line;
                while ((line = bufferedReader.readLine()) != null) {
                    if (line.trim().isEmpty()) {
                        continue;
                    }

                    final JsonObject parsed_doc = Json.parse(line).asObject();
                    final String url = parsed_doc.get("url").asString();

                    final String urlPrefix = "https://en.wikipedia.org/wiki?curid=";
                    if (url.startsWith(urlPrefix)) {
                        try {
                            final long doc_id = Long.parseLong(url.substring(urlPrefix.length()));
                            final String title = parsed_doc.get("title").asString();
                            final String body = parsed_doc.get("body").asString();

                            document.clear();

                            document.add(new LongPoint("id", doc_id));
                            document.add(new StoredField("id", doc_id));
                            document.add(new TextField("title", title, Field.Store.YES));
                            document.add(new TextField("all", title + "\n" + body, Field.Store.NO));

                            writer.addDocument(document);
                        } catch (NumberFormatException e) {
                            continue;
                        }
                    }
                }
            }

            writer.commit();
        }
    }
}
