import CompilerError from './compiler-error';

function fail(message, fileName) {
  throw new CompilerError(message, fileName);
}

function generate({ asts, entryExports, options = {} }) {
  let indent = 0;
  let output = '';

  if (!asts || !asts.length) {
    fail('No syntax trees provided to code generation step.');
  }

  function generateIndent() {
    for (let i = indent; i > 0; i--) {
      output += '  ';
    }
  }

  function generateIdentifier(node) {
    output += node.value;
  }

  function generateAssignment(node) {
    generateIndent();
    generateIdentifier(node.id);
    output += ' = ';
    generateNode(node.value);
    output += ';\n\n';
  }

  function generateBoolean(node) {
    output += node.value;
  }

  function generateFunction(node) {
    output += 'function(';
    generateIdentifier(node.parameter);
    output += ') {\n';
    indent++;
    generateIndent();
    output += 'return ';
    generateNode(node.body);
    output += ';\n';
    indent--;
    generateIndent();
    output += '}';
  }

  function generateString(node) {
    output += '"' + node.value + '"';
  }

  function generateNumber(node) {
    output += node.value + '';
  }

  function generateRecord(node) {
    output += '{ ';
    node.properties.forEach((prop, i) => {
      generateIdentifier(prop.key);
      output += ': ';
      generateNode(prop.value);
      if (i < node.properties.length - 1) output += ', ';
    });
    output += ' }';
  }

  function generateInterpolatedString(node) {
    output += '(';
    generateString(node.literals[0]);
    for (let i = 0; i < node.expressions.length; i++) {
      output += ' + ';
      generateNode(node.expressions[i]);
      const lit = node.literals[i + 1];
      if (lit.value) {
        output += ' + ';
        generateString(lit);
      }
    }
    output += ')';
  }

  function generateCall(node) {
    generateNode(node.callee);
    output += '(';
    generateNode(node.argument);
    output += ')';
  }

  function generateConditional(node) {
    output += '(';
    generateNode(node.predicate);
    output += ' ? ';
    generateNode(node.thenCase);
    output += ' : ';
    generateNode(node.elseCase);
    output += ')';
  }

  function generateNode(node) {
    switch (node.kind) {
      case 'Assignment':
        return generateAssignment(node);
      case 'Boolean':
        return generateBoolean(node);
      case 'Call':
        return generateCall(node);
      case 'Conditional':
        return generateConditional(node);
      case 'Function':
        return generateFunction(node);
      case 'Identifier':
        return generateIdentifier(node);
      case 'InterpolatedString':
        return generateInterpolatedString(node);
      case 'String':
        return generateString(node);
      case 'Number':
        return generateNumber(node);
      case 'Record':
        return generateRecord(node);
      default:
        throw 'No case for node of kind ' + node.kind;
    }
  }

  function generateExports(exportNodes = []) {
    generateIndent();
    output += 'return {';
    exportNodes.forEach((node, i) => {
      output += ' ';
      generateIdentifier(node);
      output += ': ';
      generateIdentifier(node);
      if (i < exportNodes.length - 1) output += ',';
      else output += ' ';
    });
    output += '};\n';
  }

  function generateModule(moduleNode) {
    const moduleName = moduleNode.moduleName || moduleNode.name.value;
    output += `var module$${moduleName} = (function() {\n`;

    indent++;
    generateIndent();

    const assignedVariableNames = moduleNode.body
      .filter(node => node.kind === 'Assignment')
      .map(node => node.id.value)
      .join(', ');

    if (assignedVariableNames) {
      output += 'var ' + assignedVariableNames + ';\n\n';
    }

    moduleNode.body.forEach(node => {
      generateNode(node);
      if (!/;[\s]*$/.test(output)) output += ';\n\n';
    });

    generateExports(moduleNode.exports);

    indent--;

    output += '})();';
  }

  output += `(function(global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? module.exports = factory() :
  typeof define === 'function' && define.amd ? define(factory) :
  (global.Pluma = factory());
})(this, function() { 'use strict';\n\n`;

  asts.map(generateModule);

  if (entryExports) {
    output += `\n\nreturn ${entryExports};`;
  }

  output += '\n});';

  return output;
}

export default generate;
