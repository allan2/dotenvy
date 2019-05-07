'''
Created on Aug 3, 2016

@author: YLin2
'''

import sys
import os
from xml.dom import minidom

def fix_class(class_node):
    valid_lines = 0
    covered_lines = 0
    for lines_node in class_node.getElementsByTagName('lines'):
        for line in lines_node.getElementsByTagName('line'):
            if not line.hasAttribute('hits'):
                continue
            valid_lines += 1
            hit = line.getAttribute('hits')
            if hit == '1':
                covered_lines += 1
    if valid_lines > 0:
        class_node.setAttribute('line-rate', repr(float(covered_lines)/float(valid_lines)))
    return valid_lines, covered_lines


def fix_package(package_node):
    valid_lines = 0
    covered_lines = 0
    for classes_node in package_node.getElementsByTagName('classes'):
            for class_node in classes_node.getElementsByTagName('class'):
                current_valid_lines, current_covered_lines = fix_class(class_node)
                valid_lines += current_valid_lines
                covered_lines += current_covered_lines
    if valid_lines > 0:
        package_node.setAttribute('line-rate', repr(float(covered_lines)/float(valid_lines)))
    return valid_lines, covered_lines


def fix(*args, **kargs):
    default_file_path = ''
    default_file_name = 'cobertura.xml'
    if len(args[0]) > 1:
        arg = args[0][1]
    else:
        arg = default_file_path

    if os.path.isdir(arg):
        file_name = os.path.join(arg, default_file_name)
    else:
        file_name = os.path.join(default_file_path, default_file_name)
        
    print 'processing: '+file_name
    xml_file = open(file_name, 'r')
    xml_doc = minidom.parse(xml_file)
    xml_file.close()
    xml_root = xml_doc.documentElement
    original_copy = open('coverage.original.xml', 'w')
    xml_root.writexml(original_copy)
    valid_lines = 0
    covered_lines = 0
    tag_valid_lines = 'lines-valid'
    tag_covered_lines = 'lines-covered'
    
    for package_node in xml_doc.getElementsByTagName('package'):
        current_valid_lines, current_covered_lines = fix_package(package_node)
        valid_lines += current_valid_lines
        covered_lines += current_covered_lines
        
    xml_root.setAttribute(tag_valid_lines, repr(valid_lines))
    xml_root.setAttribute(tag_covered_lines, repr(covered_lines))
    fixed_copy = open(os.path.basename(file_name), 'w')
    xml_root.writexml(fixed_copy)
        
if __name__ == '__main__':
    fix(sys.argv)

