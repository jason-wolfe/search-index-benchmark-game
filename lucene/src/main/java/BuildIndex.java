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

                LongPoint idPoint = new LongPoint("id", 0L);
                StoredField idField = new StoredField("id", 0L);
                TextField titleField = new TextField("title", "", Field.Store.YES);
                TextField allField = new TextField("all", "", Field.Store.NO);
                document.add(idPoint);
                document.add(idField);
                document.add(titleField);
                document.add(allField);

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
                            idPoint.setLongValue(doc_id);
                            idField.setLongValue(doc_id);
                            final String title = parsed_doc.get("title").asString();
                            titleField.setStringValue(title);
                            final String body = parsed_doc.get("body").asString();
                            allField.setStringValue(title + "\n" + body);
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
