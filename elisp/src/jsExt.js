////
// Emacs Lisp implementation in JavaScript.
// 
// Copyright (c) 2009 Sami Samhuri - sami.samhuri@gmail.com
//
// Released under the terms of the MIT license.  See the included file
// LICENSE.

// Simple inheritance. e.g. ChildObject.extend(ParentObject)
Function.prototype.extend = function(superclass) {
    this.prototype = new superclass;
    this.prototype.constructor = this;
};
