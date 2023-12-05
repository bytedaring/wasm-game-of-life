# build wasm npm package
build:
    @wasm-pack build --target web
# upgrade npm to www project
upgrade:
    @cd www && yarn upgrade wasm-game-of-life
# start www: yarn run dev
@dev:
    cd www && yarn upgrade wasm-game-of-life && yarn run dev
