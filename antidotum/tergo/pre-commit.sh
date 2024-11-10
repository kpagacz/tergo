cd ./src/rust/ && sh ./vendor.sh && cd ../../
Rscript -e "rextendr::document()"
Rscript -e "devtools::document()"
Rscript update_authors.R
